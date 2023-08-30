import React, { memo, useCallback, useEffect, useRef, useState } from "react";
import * as inputStyles from "@paperclip-ui/designer/src/styles/input.pc";
import { useDispatch, useSelector } from "@paperclip-ui/common";
import { DesignerEvent } from "@paperclip-ui/designer/src/events";
import { DeclarationValue } from "./DeclarationValue";
import { NewDeclValue } from "./types";
import { noop } from "lodash";
import { getPropField } from "./cssSchema";
import { SelectDetails } from "@paperclip-ui/designer/src/components/SuggestionMenu";
import {
  ComputedStyle,
  serializeDeclaration,
} from "@paperclip-ui/proto-ext/lib/ast/serialize";
import { NameInput } from "./NameInput";
import { getTargetExprId } from "@paperclip-ui/designer/src/state";
// import { InteractiveDeclValue } from "./InteractiveDeclValue";

type FieldProps = {
  name?: string;
  style?: ComputedStyle;
  onFocus?: () => void;
  onBlur?: () => void;
  onValueTab?: (event: React.KeyboardEvent<any>) => void;
  isNew?: boolean;
};

export const Declaration = memo(
  ({
    name,
    style,
    onBlur = noop,
    onFocus = noop,
    onValueTab = noop,
    isNew,
  }: FieldProps) => {
    const value = style && serializeDeclaration(style.value);
    const [name2, setName] = useState(name);
    const { input: inputOptions } = getPropField(name2);
    const dispatch = useDispatch<DesignerEvent>();
    const targetId = useSelector(getTargetExprId);
    const ref = useRef(null);

    useEffect(() => {
      setName(name);
    }, [name]);

    const onValueSave = useCallback(
      (value: string, imports: Record<string, string>) => {
        console.log("SAVE");
        dispatch({
          type: "ui/styleDeclarationsChanged",
          payload: {
            values: { [name2]: value },
            imports,
          },
        });
      },
      [name2]
    );

    const nameInput = isNew ? (
      <NameInput name={name2} onSave={setName} />
    ) : (
      name
    );

    const input = (
      <DeclarationValue
        name={name2}
        value={value}
        isInherited={style?.ownerId !== targetId}
        onTab={onValueTab}
        onSave={onValueSave}
      />
    );

    const onBlur2 = () => {
      setTimeout(() => {
        if (ref.current.contains(document.activeElement)) {
          return;
        }
        onBlur();
      });
    };

    // default field input
    return (
      <inputStyles.Field
        ref={ref}
        onFocus={onFocus}
        onBlur={onBlur2}
        name={nameInput}
        input={input}
      />
    );
  }
);
