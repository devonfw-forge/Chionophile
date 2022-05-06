"use strict";
var __decorate = (this && this.__decorate) || function (decorators, target, key, desc) {
    var c = arguments.length, r = c < 3 ? target : desc === null ? desc = Object.getOwnPropertyDescriptor(target, key) : desc, d;
    if (typeof Reflect === "object" && typeof Reflect.decorate === "function") r = Reflect.decorate(decorators, target, key, desc);
    else for (var i = decorators.length - 1; i >= 0; i--) if (d = decorators[i]) r = (c < 3 ? d(r) : c > 3 ? d(target, key, r) : d(target, key)) || r;
    return c > 3 && r && Object.defineProperty(target, key, r), r;
};
var __metadata = (this && this.__metadata) || function (k, v) {
    if (typeof Reflect === "object" && typeof Reflect.metadata === "function") return Reflect.metadata(k, v);
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.ComposedCTO = void 0;
const queue_entity_1 = require("../../queue/model/entities/queue.entity");
const visitor_entity_1 = require("../../visitor/model/entities/visitor.entity");
const accessCodeResponse_1 = require("./accessCodeResponse");
const class_transformer_1 = require("class-transformer");
let ComposedCTO = class ComposedCTO {
};
__decorate([
    (0, class_transformer_1.Expose)(),
    __metadata("design:type", accessCodeResponse_1.AccessCodeResponse)
], ComposedCTO.prototype, "accessCode", void 0);
__decorate([
    (0, class_transformer_1.Expose)(),
    __metadata("design:type", queue_entity_1.Queue)
], ComposedCTO.prototype, "queue", void 0);
__decorate([
    (0, class_transformer_1.Expose)(),
    __metadata("design:type", visitor_entity_1.Visitor)
], ComposedCTO.prototype, "visitor", void 0);
ComposedCTO = __decorate([
    (0, class_transformer_1.Exclude)()
], ComposedCTO);
exports.ComposedCTO = ComposedCTO;
//# sourceMappingURL=composedCto.js.map