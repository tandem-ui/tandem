import { memoize } from "@paperclip-ui/common";
import { DeclarationValue, StyleDeclaration } from "../generated/ast/css";
import { Graph } from "../generated/ast/graph";
import {
  Component,
  ComponentBodyItem,
  Document,
  DocumentBodyItem,
  Element,
  Import,
  Insert,
  Node,
  Reference,
  Render,
  Slot,
  Style,
  TextNode,
} from "../generated/ast/pc";

const EMPTY_ARRAY = [];
export namespace ast {
  export type InnerNode = Element | Insert | Slot | TextNode;
  export type InnerExpression = Document | InnerNode | Style;

  export const getDocumentBodyInner = (item: DocumentBodyItem) => {
    // oneof forces us to do this :(
    return (
      item.atom ||
      item.component ||
      item.docComment ||
      item.element ||
      item.import ||
      item.style ||
      item.text ||
      item.trigger
    );
  };

  export const getNodeInner = (item: Node) => {
    // oneof forces us to do this :(
    return (
      item.element ||
      item.insert ||
      item.override ||
      item.slot ||
      item.style ||
      item.text
    );
  };

  export const getComponentBodyInner = (item: ComponentBodyItem) => {
    // oneof forces us to do this :(
    return item.render || item.script || item.variant;
  };

  export const getInnerExpression = (
    item: DocumentBodyItem | Node | ComponentBodyItem
  ) =>
    getNodeInner(item as DocumentBodyItem) ||
    getDocumentBodyInner(item as Node) ||
    getComponentBodyInner(item as ComponentBodyItem);
  export const getChildren = (expr: InnerExpression) =>
    ((expr as Document | InnerNode).body as Array<DocumentBodyItem | Node>) ||
    EMPTY_ARRAY;

  export const getAncestorIds = memoize((id: string, graph: Graph) => {
    const ancestorIds: string[] = [];

    const dep = getOwnerDependency(id, graph);
    const exprsById = flattenDocument(dep.document);
    const childParentMap = getChildParentMap(exprsById);

    let curr = exprsById[id];

    while (curr) {
      const nextId = childParentMap[curr.id];
      const next = exprsById[nextId];

      if (next) {
        ancestorIds.push(next.id);
      }
      curr = next;
    }

    return ancestorIds;
  });

  export const getAncestorVirtIdsFromShadow = memoize(
    (id: string, graph: Graph) => {
      const instanceIds = id.split(".");
      const ancestorIds = [];

      for (let i = instanceIds.length; i--; ) {
        const targetId = instanceIds[i];
        const targetAncestorIds = [...getAncestorIds(targetId, graph)];

        if (i !== instanceIds.length - 1) {
          targetAncestorIds.unshift(targetId);
        }

        for (const id of targetAncestorIds) {
          ancestorIds.push([...instanceIds.slice(0, i), id].join("."));
        }
      }

      return ancestorIds;
    }
  );

  export const getShadowExprId = (id: string) => id.split(".").pop();

  export const getChildParentMap = memoize(
    (exprs: Record<string, InnerExpression>) => {
      const map: Record<string, string> = {};

      for (const id in exprs) {
        for (const child of getChildren(exprs[id])) {
          map[getInnerExpression(child).id] = id;
        }
      }
      return map;
    }
  );

  export const getRef = (ref: Reference, graph: Graph): Style | null => {
    const dep = getOwnerDependency(ref.id, graph);
    if (ref.path.length === 1) {
      return getDocumentBodyStyleByName(ref.path[0], dep.document);
    } else {
      // const imp = graph.dependencies[dep.imports[]];
      const imp = getDocumentImport(ref.path[0], dep.document);
      const impDep = imp && graph.dependencies[imp.path];

      // Broken references will happen all the time
      if (impDep) {
        return getDocumentBodyStyleByName(ref.path[1], impDep.document);
      }
    }
    return null;
  };

  export const getDocumentBodyStyleByName = (
    name: string,
    document: Document
  ) => {
    return document.body.find((expr) => expr.style?.name === name) as Style;
  };

  export const computeElementStyle = memoize(
    (
      exprId: string,
      graph: Graph,
      variantIds?: string[]
    ): Record<string, DeclarationValue> => {
      // TODO
      const node = getExprById(exprId.split(".").pop(), graph);
      const map: Record<string, DeclarationValue> = {};
      for (const item of node.body) {
        const { style } = item;

        if (style) {
          Object.assign({}, computeStyle(style, graph, variantIds));
        }
      }

      return map;
    }
  );

  export const computeStyle = memoize(
    (
      style: Style,
      graph: Graph,
      variantIds?: string[]
    ): Record<string, DeclarationValue> => {
      let map: Record<string, DeclarationValue> = {};

      if (style.variantCombo && style.variantCombo.length > 0) {
        // TODO: do ehthis
        // if (!style.variantCombo.every(ref => getRef))
        return map;
      }

      for (const value of style.declarations) {
        map[value.name] = value.value;
      }

      if (style.extends) {
        for (const ref of style.extends) {
          const extendsStyle = getRef(ref, graph);
          if (extendsStyle) {
            map = Object.assign(
              {},
              computeStyle(extendsStyle, graph, variantIds),
              map
            );
          }
        }
      }

      return map;
    }
  );

  export const getComponentRenderNode = (component: Component) =>
    component.body.find((body) => body.render).render;

  export const isInstance = (element: Element, graph: Graph) => {
    return getInstanceComponent(element, graph) != null;
  };

  export const isComponent = (expr: InnerExpression): expr is Component => {
    return (
      (expr as Component).name != null &&
      (expr as Component).body?.some((expr) => expr.render != null)
    );
  };

  export const getInstanceComponent = (element: Element, graph: Graph) => {
    return getDocumentComponent(
      element.tagName,
      getInstanceDefinitionDependency(element, graph).document
    );
  };

  export const getInstanceDefinitionDependency = (
    element: Element,
    graph: Graph
  ) => {
    const instanceDependency = getOwnerDependency(element.id, graph);
    const documentImport =
      element.namespace &&
      getDocumentImport(element.namespace, instanceDependency.document);
    if (documentImport) {
      return graph.dependencies[
        instanceDependency.imports[documentImport.path]
      ];
    } else {
      return instanceDependency;
    }
  };

  export const getDocumentComponents = memoize(
    (document: Document): Component[] =>
      document.body
        .filter((body) => body.component != null)
        .map(getDocumentBodyInner) as Component[]
  );
  export const getDocumentComponent = (
    name: string,
    document: Document
  ): Component =>
    getDocumentComponents(document).find(
      (component) => component.name === name
    );

  export const getDocumentImports = memoize(
    (document: Document): Import[] =>
      document.body
        .filter((body) => body.import != null)
        .map(getDocumentBodyInner) as Import[]
  );
  export const getDocumentImport = (
    namespace: string,
    document: Document
  ): Import =>
    getDocumentImports(document).find((imp) => imp.namespace === namespace);

  export const getOwnerDependencyPath = memoize(
    (exprId: string, graph: Graph) => {
      for (const path in graph.dependencies) {
        const dep = graph.dependencies[path];
        if (containsExpression(exprId, dep.document)) {
          return path;
        }
      }
      return null;
    }
  );

  export const getOwnerDependency = (exprId: string, graph: Graph) => {
    return graph.dependencies[getOwnerDependencyPath(exprId, graph)];
  };

  export const containsExpression = (
    exprId: string,
    ancestor: InnerExpression
  ) => {
    return flattenUnknownInnerExpression(ancestor)[exprId] != null;
  };

  export const getExprByVirtId = (id: string, graph: Graph) =>
    getExprById(id.split(".").pop(), graph);
  export const getExprStyles = (
    parent: Element | TextNode | Document
  ): Style[] =>
    (parent as Element).body
      .filter((expr) => expr.style)
      .map(getInnerExpression);

  export const getExprById = (id: string, graph: Graph) => {
    return flattenDocument(getOwnerDependency(id, graph).document)[id];
  };

  export const flattenUnknownInnerExpression = memoize(
    (expr: Document | Node): Record<string, InnerExpression> =>
      Object.assign(flattenDocument(expr as Document), flattenElement(expr))
  );

  export const flattenDocument = memoize((expr: Document) => {
    return Object.assign(
      {
        [expr.id]: expr,
      },
      ...expr.body.map(flattenDocumentBodyItem)
    );
  });

  export const flattenDocumentBodyItem = (expr: DocumentBodyItem) => {
    if (expr.element) {
      return flattenElement(expr.element);
    }
    if (expr.text) {
      return flattenTextNode(expr.text);
    }
    if (expr.component) {
      return flattenComponent(expr.component);
    }
    return {};
  };

  export const flattenElement = memoize((expr: Element) => {
    return Object.assign(
      {
        [expr.id]: expr,
      },
      ...expr.body.map(flattenNode)
    );
  });

  export const flattenComponent = memoize((expr: Component) => {
    return Object.assign(
      {
        [expr.id]: expr,
      },
      ...expr.body.map(flattenComponentBodyItem)
    );
  });

  export const flattenComponentBodyItem = memoize((expr: ComponentBodyItem) => {
    if (expr.render) {
      return flattenRender(expr.render);
    }
    return {};
  });

  export const flattenRender = memoize((expr: Render) => {
    return Object.assign(
      {
        [expr.id]: expr,
      },
      flattenNode(expr.node)
    );
  });

  export const flattenTextNode = memoize((expr: TextNode) => {
    return {
      [expr.id]: expr,
    };
  });

  export const flattenSlot = memoize((expr: Slot) => {
    return Object.assign(
      {
        [expr.id]: expr,
      },
      ...expr.body.map(flattenNode)
    );
  });

  export const flattenInsert = memoize((expr: Insert) => {
    return Object.assign(
      {
        [expr.id]: expr,
      },
      ...expr.body.map(flattenNode)
    );
  });

  export const flattenStyle = memoize((expr: Style) => {
    return Object.assign(
      {
        [expr.id]: expr,
      },
      ...expr.declarations.map(flattenDeclaration),
      ...(expr.extends || []).map(flattenReference)
    );
  });

  export const flattenDeclaration = memoize((expr: StyleDeclaration) => {
    return Object.assign({
      [expr.id]: expr,
    });
  });
  export const flattenReference = memoize((expr: Reference) => ({
    [expr.id]: expr,
  }));

  export const flattenNode = (expr: Node) => {
    if (expr.element) {
      return flattenElement(expr.element);
    }
    if (expr.style) {
      return flattenStyle(expr.style);
    }

    if (expr.text) {
      return flattenTextNode(expr.text);
    }

    if (expr.slot) {
      return flattenSlot(expr.slot);
    }

    if (expr.insert) {
      return flattenInsert(expr.insert);
    }

    return {};
  };
}
