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
exports.Queue = void 0;
const openapi = require("@nestjs/swagger");
const typeorm_1 = require("typeorm");
const crud_1 = require("@nestjsx/crud");
const class_validator_1 = require("class-validator");
const accessCode_entity_1 = require("../../../accesscode/model/entities/accessCode.entity");
const class_transformer_1 = require("class-transformer");
let Queue = class Queue {
    constructor() {
        this.modificationCounter = 1;
        this.attentionTime = new Date();
    }
    static _OPENAPI_METADATA_FACTORY() {
        return { id: { required: true, type: () => Number }, modificationCounter: { required: false, type: () => Number, default: 1 }, name: { required: false, type: () => String, maxLength: 255 }, logo: { required: false, type: () => String, maxLength: 255 }, currentNumber: { required: false, type: () => String, maxLength: 255 }, attentionTime: { required: true, type: () => Date, default: new Date() }, minAttentionTime: { required: false, type: () => Date }, active: { required: false, type: () => Boolean }, accessCodes: { required: true, type: () => [require("../../../accesscode/model/entities/accessCode.entity").AccessCode] } };
    }
};
__decorate([
    (0, class_transformer_1.Transform)(({ value }) => parseInt(value)),
    (0, typeorm_1.PrimaryGeneratedColumn)(),
    __metadata("design:type", Number)
], Queue.prototype, "id", void 0);
__decorate([
    (0, typeorm_1.Column)('int', { name: 'modificationcounter' }),
    __metadata("design:type", Number)
], Queue.prototype, "modificationCounter", void 0);
__decorate([
    (0, class_validator_1.IsDefined)({ groups: [crud_1.CrudValidationGroups.CREATE] }),
    (0, class_validator_1.IsOptional)({ groups: [crud_1.CrudValidationGroups.UPDATE] }),
    (0, class_validator_1.MaxLength)(255),
    (0, typeorm_1.Column)('varchar', { length: 255, nullable: true }),
    __metadata("design:type", String)
], Queue.prototype, "name", void 0);
__decorate([
    (0, class_validator_1.IsDefined)({ groups: [crud_1.CrudValidationGroups.CREATE] }),
    (0, class_validator_1.IsOptional)({ groups: [crud_1.CrudValidationGroups.UPDATE] }),
    (0, class_validator_1.MaxLength)(255),
    (0, typeorm_1.Column)('varchar', { length: 255, nullable: true }),
    __metadata("design:type", String)
], Queue.prototype, "logo", void 0);
__decorate([
    (0, class_validator_1.IsDefined)({ groups: [crud_1.CrudValidationGroups.CREATE] }),
    (0, class_validator_1.IsOptional)({ groups: [crud_1.CrudValidationGroups.UPDATE] }),
    (0, class_validator_1.MaxLength)(255),
    (0, typeorm_1.Column)('varchar', { length: 255, name: 'currentnumber' }),
    __metadata("design:type", String)
], Queue.prototype, "currentNumber", void 0);
__decorate([
    (0, typeorm_1.Column)('timestamp', { name: 'attentiontime' }),
    __metadata("design:type", Date)
], Queue.prototype, "attentionTime", void 0);
__decorate([
    (0, class_validator_1.IsDefined)({ groups: [crud_1.CrudValidationGroups.CREATE] }),
    (0, class_validator_1.IsOptional)({ groups: [crud_1.CrudValidationGroups.UPDATE] }),
    (0, typeorm_1.Column)('timestamp', { name: 'minattentiontime' }),
    __metadata("design:type", Date)
], Queue.prototype, "minAttentionTime", void 0);
__decorate([
    (0, class_validator_1.IsDefined)({ groups: [crud_1.CrudValidationGroups.CREATE] }),
    (0, class_validator_1.IsOptional)({ groups: [crud_1.CrudValidationGroups.UPDATE] }),
    (0, typeorm_1.Column)('bool'),
    __metadata("design:type", Boolean)
], Queue.prototype, "active", void 0);
__decorate([
    (0, typeorm_1.OneToMany)(() => accessCode_entity_1.AccessCode, accessCode => accessCode.queue),
    __metadata("design:type", Array)
], Queue.prototype, "accessCodes", void 0);
Queue = __decorate([
    (0, typeorm_1.Entity)({ name: 'dailyqueue' })
], Queue);
exports.Queue = Queue;
//# sourceMappingURL=queue.entity.js.map