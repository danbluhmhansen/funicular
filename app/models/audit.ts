export enum AuditState {
  Unchanged,
  Added,
  Updated,
  Removed,
}

export interface Audit<T> {
  state: AuditState;
  ante?: T | undefined;
  post?: T | undefined;
}

export function auditTrack<T>(value: T): Audit<T> {
  return {
    state: AuditState.Unchanged,
    ante: value,
    post: value,
  };
}

export function auditAdd<T>(value: T): Audit<T> {
  return {
    state: AuditState.Added,
    post: value,
  };
}
