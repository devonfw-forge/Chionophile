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
var __param = (this && this.__param) || function (paramIndex, decorator) {
    return function (target, key) { decorator(target, key, paramIndex); }
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.QueueCrudController = void 0;
const openapi = require("@nestjs/swagger");
const common_1 = require("@nestjs/common");
const crud_1 = require("@nestjsx/crud");
const serializer_1 = require("@devon4node/common/serializer");
const queue_entity_1 = require("../model/entities/queue.entity");
const queue_crud_service_1 = require("../services/queue.crud.service");
const swagger_1 = require("@nestjs/swagger");
const class_transformer_1 = require("class-transformer");
let QueueCrudController = class QueueCrudController {
    constructor(service) {
        this.service = service;
    }
    async create(req, dto, res) {
        const resData = (0, class_transformer_1.plainToClass)(queue_entity_1.Queue, await this.service.createOne(req, dto));
        res.status(common_1.HttpStatus.OK).send(resData);
    }
    async get(req, res) {
        try {
            const resData = (0, class_transformer_1.plainToClass)(queue_entity_1.Queue, await this.service.getOne(req));
            res.status(common_1.HttpStatus.OK).send(resData);
        }
        catch (_a) {
            res.status(common_1.HttpStatus.NOT_FOUND).send();
        }
    }
    async delete(req, id, res) {
        try {
            await this.service.deleteOne(req);
            res.status(common_1.HttpStatus.OK).json(id);
        }
        catch (_a) {
            res.status(common_1.HttpStatus.NOT_FOUND).send();
        }
    }
};
__decorate([
    (0, crud_1.Override)('createOneBase'),
    openapi.ApiResponse({ status: 200 }),
    __param(0, (0, crud_1.ParsedRequest)()),
    __param(1, (0, crud_1.ParsedBody)()),
    __param(2, (0, common_1.Res)()),
    __metadata("design:type", Function),
    __metadata("design:paramtypes", [Object, queue_entity_1.Queue, Object]),
    __metadata("design:returntype", Promise)
], QueueCrudController.prototype, "create", null);
__decorate([
    (0, crud_1.Override)('getOneBase'),
    openapi.ApiResponse({ status: 200 }),
    __param(0, (0, crud_1.ParsedRequest)()),
    __param(1, (0, common_1.Res)()),
    __metadata("design:type", Function),
    __metadata("design:paramtypes", [Object, Object]),
    __metadata("design:returntype", Promise)
], QueueCrudController.prototype, "get", null);
__decorate([
    (0, crud_1.Override)('deleteOneBase'),
    openapi.ApiResponse({ status: 200 }),
    __param(0, (0, crud_1.ParsedRequest)()),
    __param(1, (0, common_1.Param)('id')),
    __param(2, (0, common_1.Res)()),
    __metadata("design:type", Function),
    __metadata("design:paramtypes", [Object, Number, Object]),
    __metadata("design:returntype", Promise)
], QueueCrudController.prototype, "delete", null);
QueueCrudController = __decorate([
    (0, crud_1.Crud)({
        model: {
            type: queue_entity_1.Queue,
        },
    }),
    (0, serializer_1.CrudType)(queue_entity_1.Queue),
    (0, common_1.Controller)('queuemanagement/v1/queue'),
    (0, swagger_1.ApiTags)('Queue'),
    __metadata("design:paramtypes", [queue_crud_service_1.QueueCrudService])
], QueueCrudController);
exports.QueueCrudController = QueueCrudController;
//# sourceMappingURL=queue.crud.controller.js.map