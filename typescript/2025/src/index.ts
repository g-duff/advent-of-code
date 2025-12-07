export type Option<T> = T | undefined;
export const defined = <T>(opt: Option<T>) => opt !== undefined;
