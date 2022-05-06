"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.BusinessLogicException = void 0;
class BusinessLogicException extends Error {
    constructor(message, errorId) {
        super();
        this._name = 'BusinessLogicException';
        this._message = message;
        this._errorId = errorId;
    }
    get errorId() {
        return this._errorId;
    }
    get name() {
        return this._name;
    }
    get message() {
        return this._message;
    }
    plainObject() {
        return { message: this.message, name: this.name, errorId: this.errorId };
    }
}
exports.BusinessLogicException = BusinessLogicException;
//# sourceMappingURL=business-logic.exception.js.map