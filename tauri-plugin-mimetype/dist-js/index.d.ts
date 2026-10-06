/** Map a file extension (e.g. "json", "html") to a MIME type. Returns null if unknown. */
export declare function fromExtension(extension: string): Promise<string | null>;
/** Map a file path (e.g. "/path/to/file.json") to a MIME type via its extension. Returns null if unknown. */
export declare function fromPath(path: string): Promise<string | null>;
