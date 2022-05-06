export declare class BusinessLogicException extends Error {
    private readonly _errorId?;
    private readonly _name;
    private readonly _message;
    constructor(message: string, errorId?: string);
    get errorId(): string | undefined;
    get name(): string;
    get message(): string;
    plainObject(): Pick<BusinessLogicException, 'message' | 'name' | 'errorId'>;
}
