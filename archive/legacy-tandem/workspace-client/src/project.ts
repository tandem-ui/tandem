import { RPCClientAdapter } from "@paperclip-lang/common";
import {
  getAllPaperclipFilesChannel,
  loadProjectInfoChannel,
  openProjectChannel,
  searchProjectChannel,
} from "@tandem-ui/workspace-core";
import { PaperclipManager } from "./paperclip";

export type LoadOptions = {
  id?: string;
  uri?: string;
};

export type ProjectProperties = {
  id: string;
  directoryPath: string;
  directoryUri: string;
};

export class Project {
  private _properties: ProjectProperties;
  private _paperclip: PaperclipManager;
  private _openProject: ReturnType<typeof openProjectChannel>;
  private _getAllPaperclipFiles: ReturnType<typeof getAllPaperclipFilesChannel>;
  private _loadProjectChannel: ReturnType<typeof loadProjectInfoChannel>;
  private _searchProject: ReturnType<typeof searchProjectChannel>;

  /**
   */

  private constructor(
    private _loadOptions: LoadOptions,
    // private _editorClient: EditorClient,
    private _client: RPCClientAdapter
  ) {
    this._openProject = openProjectChannel(_client);
    this._getAllPaperclipFiles = getAllPaperclipFilesChannel(_client);
    this._loadProjectChannel = loadProjectInfoChannel(_client);
    this._searchProject = searchProjectChannel(_client);
    this._paperclip = new PaperclipManager(this._client);
  }

  /**
   */

  async getInfo() {
    return await this._loadProjectChannel.call({
      projectId: this._properties.id,
    });
  }

  /**
   */

  search(filterText: string) {
    return this._searchProject.call({
      filterText,
      projectId: this._properties.id,
    });
  }

  /**
   */

  getProperties() {
    return this._properties;
  }

  /**
   */

  getPaperclip() {
    return this._paperclip;
  }

  /**
   */

  static async load(
    options: LoadOptions,
    // documents: EditorClient,
    client: RPCClientAdapter
  ) {
    const project = new Project(options, client);
    await project._open();
    return project;
  }

  /**
   */

  // getDocuments() {
  //   return this._editorClient.getDocuments();
  // }

  /**
   */

  private async _open() {
    this._properties = await this._openProject.call(this._loadOptions);
  }

  /**
   */

  // async openAllPaperclipDocuments() {
  //   const fileUris = await this._getAllPaperclipFiles.call({
  //     projectId: this._properties.id,
  //   });
  //   const docs: PCDocument[] = await Promise.all(
  //     fileUris.map((uri) => {
  //       return this._editorClient.getDocuments().open(uri);
  //     })
  //   );
  //   return docs;
  // }
}