export enum Status {
    Success = "success",
    Error = "error",
}

export interface Response {
    status: Status;
    result: string;
}