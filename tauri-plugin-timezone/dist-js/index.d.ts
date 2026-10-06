/**
 * Returns the system's IANA timezone identifier (e.g. "Europe/Madrid", "America/New_York").
 * Returns null if the timezone cannot be determined.
 */
export declare function getTimezone(): Promise<string | null>;
/**
 * Returns the current UTC offset in seconds (east of UTC is positive).
 * DST-aware: Europe/Madrid returns 7200 in summer (UTC+2), 3600 in winter (UTC+1).
 * Note: this is the OPPOSITE sign convention from `Date.prototype.getTimezoneOffset()`,
 * which returns minutes west of UTC (negative east of UTC).
 */
export declare function getUtcOffsetSeconds(): Promise<number>;
