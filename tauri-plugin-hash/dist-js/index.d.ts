export type HashAlgorithm = 'sha256' | 'sha512' | 'md5';
/** Compute the hash of a UTF-8 string. Returns the hex-encoded digest. */
export declare function hashString(input: string, algorithm: HashAlgorithm): Promise<string>;
/** Compute the hash of a file's contents (streamed in 8 KB chunks). Returns the hex-encoded digest. */
export declare function hashFile(path: string, algorithm: HashAlgorithm): Promise<string>;
