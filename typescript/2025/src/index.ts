export type Option<T> = T | undefined;
export const isDefined = <T>(opt: Option<T>) => opt !== undefined;
