/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */

import { glob } from 'glob';
import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';

import {
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	TransportKind
} from 'vscode-languageclient/node';

let coreClient: LanguageClient;
let foundryClient: LanguageClient;

export async function activate(context: ExtensionContext) {
	// The server is implemented in node
	const coreServerModule = context.asAbsolutePath(
		path.join('dist', 'server.js')
	);

	// If the extension is launched in debug mode then the debug server options are used
	// Otherwise the run options are used
	const coreServerOptions: ServerOptions = {
		run: { module: coreServerModule, transport: TransportKind.ipc },
		debug: {
			module: coreServerModule,
			transport: TransportKind.ipc,
		}
	};

	// Options to control the language client
	const coreClientOptions: LanguageClientOptions = {
		// Register the server for plain text documents
		documentSelector: [{ scheme: 'file', language: 'solidity' }],
		synchronize: {
			// Notify the server about file changes to '.clientrc files contained in the workspace
			fileEvents: workspace.createFileSystemWatcher('**/.solidhunter.json')
		}
	};

	// Create the language client and start the client.
	coreClient = new LanguageClient(
		'osmium-solidity',
		'Osmium Solidity Language Server',
		coreServerOptions,
		coreClientOptions
	);

	// The server is a binary executable
	const foundryServerBinary = context.asAbsolutePath(
		path.join('dist', 'foundry-server')
	);

	const foundryServerOptions: ServerOptions = {
		run: { command: foundryServerBinary, transport: TransportKind.stdio },
		debug: { command: foundryServerBinary, transport: TransportKind.stdio }
	};

	// Options to control the language client
	const foundryClientOptions: LanguageClientOptions = {
		// Register the server for plain text documents
		documentSelector: [{ scheme: 'file', language: 'solidity' }],
		synchronize: {
			// Notify the server about file changes to '.clientrc files contained in the workspace
			fileEvents: workspace.createFileSystemWatcher('**/foundry.toml')
		}
	};

	foundryClient = new LanguageClient(
		'osmium-foundry',
		'Osmium Foundry Language Server',
		foundryServerOptions,
		foundryClientOptions
	);
	// Start the clients. This will also launch the servers
	coreClient.start();
	foundryClient.start();

	const folders = workspace.workspaceFolders;
	if (folders) {
		const folder = folders[0];
		const files = await workspace.findFiles('**/*.sol', `${folder.uri.fsPath}/**`);
		files.forEach(file => {
			workspace.openTextDocument(file);
		});
	}

}

export function deactivate(): Thenable<void> | undefined {
	if (!coreClient) {
		return undefined;
	}
	return coreClient.stop();
}
