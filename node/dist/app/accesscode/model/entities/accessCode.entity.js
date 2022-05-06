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
exports.AccessCode = void 0;
const openapi = require("@nestjs/swagger");
const typeorm_1 = require("typeorm");
const visitor_entity_1 = require("../../../visitor/model/entities/visitor.entity");
const queue_entity_1 = require("../../../queue/model/entities/queue.entity");
const class_transformer_1 = require("class-transformer");
let AccessCode = class AccessCode {
    constructor() {
        this.modificationCounter = 1;
        this.creationTime = new Date();
        this.startTime = new Date();
        this.endTime = new Date();
    }
    static _OPENAPI_METADATA_FACTORY() {
        return { id: { required: true, type: () => Number }, modificationCounter: { required: false, type: () => Number, default: 1 }, creationTime: { required: false, type: () => Date, default: new Date() }, startTime: { required: false, type: () => Date, default: new Date() }, endTime: { required: false, type: () => Date, default: new Date() }, visitorId: { required: false, type: () => Number }, queueId: { required: false, type: () => Number }, visitor: { required: true, type: () => require("../../../visitor/model/entities/visitor.entity").Visitor }, queue: { required: true, type: () => require("../../../queue/model/entities/queue.entity").Queue } };
    }
};
__decorate([
    (0, class_transformer_1.Transform)(({ value }) => parseInt(value)),
    (0, typeorm_1.PrimaryGeneratedColumn)(),
    __metadata("design:type", Number)
], AccessCode.prototype, "id", void 0);
__decorate([
    (0, typeorm_1.Column)('int', { name: 'modificationcounter', nullable: true }),
    __metadata("design:type", Number)
], AccessCode.prototype, "modificationCounter", void 0);
__decorate([
    (0, typeorm_1.Column)('timestamp', { name: 'creationtime' }),
    __metadata("design:type", Date)
], AccessCode.prototype, "creationTime", void 0);
__decorate([
    (0, typeorm_1.Column)('timestamp', { name: 'starttime' }),
    __metadata("design:type", Date)
], AccessCode.prototype, "startTime", void 0);
__decorate([
    (0, typeorm_1.Column)('timestamp', { name: 'endtime' }),
    __metadata("design:type", Date)
], AccessCode.prototype, "endTime", void 0);
__decorate([
    (0, class_transformer_1.Transform)(({ value }) => parseInt(value)),
    (0, typeorm_1.Column)('int', { name: 'idvisitor', nullable: true }),
    __metadata("design:type", Number)
], AccessCode.prototype, "visitorId", void 0);
__decorate([
    (0, class_transformer_1.Transform)(({ value }) => parseInt(value)),
    (0, typeorm_1.Column)('int', { name: 'idqueue', nullable: true }),
    __metadata("design:type", Number)
], AccessCode.prototype, "queueId", void 0);
__decorate([
    (0, typeorm_1.OneToOne)(() => visitor_entity_1.Visitor),
    (0, typeorm_1.JoinColumn)({ name: 'id' }),
    __metadata("design:type", visitor_entity_1.Visitor)
], AccessCode.prototype, "visitor", void 0);
__decorate([
    (0, typeorm_1.ManyToOne)(() => queue_entity_1.Queue, queue => queue.accessCodes),
    (0, typeorm_1.JoinColumn)({ name: 'idqueue' }),
    __metadata("design:type", queue_entity_1.Queue)
], AccessCode.prototype, "queue", void 0);
AccessCode = __decorate([
    (0, typeorm_1.Entity)({ name: 'accesscode' })
], AccessCode);
exports.AccessCode = AccessCode;
//# sourceMappingURL=accessCode.entity.js.map