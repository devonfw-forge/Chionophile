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
exports.Visitor = void 0;
const openapi = require("@nestjs/swagger");
const typeorm_1 = require("typeorm");
const class_validator_1 = require("class-validator");
const class_transformer_1 = require("class-transformer");
let Visitor = class Visitor {
    constructor() {
        this.modificationCounter = 1;
    }
    static _OPENAPI_METADATA_FACTORY() {
        return { id: { required: true, type: () => Object }, modificationCounter: { required: false, type: () => Number, default: 1 }, username: { required: true, type: () => String }, name: { required: true, type: () => String }, password: { required: true, type: () => String }, phoneNumber: { required: true, type: () => String, maxLength: 255 }, acceptedCommercial: { required: true, type: () => Boolean }, acceptedTerms: { required: true, type: () => Boolean }, userType: { required: true, type: () => Boolean } };
    }
};
__decorate([
    (0, class_transformer_1.Transform)(({ value }) => parseInt(value)),
    (0, typeorm_1.PrimaryGeneratedColumn)('increment'),
    __metadata("design:type", Object)
], Visitor.prototype, "id", void 0);
__decorate([
    (0, typeorm_1.Column)('int', { name: 'modificationcounter' }),
    __metadata("design:type", Number)
], Visitor.prototype, "modificationCounter", void 0);
__decorate([
    (0, class_validator_1.IsEmail)(),
    (0, typeorm_1.Column)('varchar', { length: 255, nullable: true }),
    __metadata("design:type", String)
], Visitor.prototype, "username", void 0);
__decorate([
    (0, typeorm_1.Column)('varchar', { length: 255, nullable: true }),
    __metadata("design:type", String)
], Visitor.prototype, "name", void 0);
__decorate([
    (0, typeorm_1.Column)('varchar', { length: 255, nullable: true }),
    __metadata("design:type", String)
], Visitor.prototype, "password", void 0);
__decorate([
    (0, class_validator_1.MaxLength)(255),
    (0, typeorm_1.Column)('varchar', { name: 'phonenumber', length: 255, nullable: true }),
    __metadata("design:type", String)
], Visitor.prototype, "phoneNumber", void 0);
__decorate([
    (0, typeorm_1.Column)('bool', { name: 'acceptedcommercial', nullable: true }),
    __metadata("design:type", Boolean)
], Visitor.prototype, "acceptedCommercial", void 0);
__decorate([
    (0, typeorm_1.Column)('bool', { name: 'acceptedterms', nullable: true }),
    __metadata("design:type", Boolean)
], Visitor.prototype, "acceptedTerms", void 0);
__decorate([
    (0, typeorm_1.Column)('bool', { name: 'usertype', nullable: true }),
    __metadata("design:type", Boolean)
], Visitor.prototype, "userType", void 0);
Visitor = __decorate([
    (0, typeorm_1.Entity)({ name: 'visitor' })
], Visitor);
exports.Visitor = Visitor;
//# sourceMappingURL=visitor.entity.js.map