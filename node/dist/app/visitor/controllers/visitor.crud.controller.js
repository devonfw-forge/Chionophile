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
exports.VisitorCrudController = void 0;
const openapi = require("@nestjs/swagger");
const common_1 = require("@nestjs/common");
const crud_1 = require("@nestjsx/crud");
const serializer_1 = require("@devon4node/common/serializer");
const visitor_entity_1 = require("../model/entities/visitor.entity");
const visitor_crud_service_1 = require("../services/visitor.crud.service");
const swagger_1 = require("@nestjs/swagger");
const visitordto_1 = require("../dto/visitordto");
const class_transformer_1 = require("class-transformer");
let VisitorCrudController = class VisitorCrudController {
    constructor(service) {
        this.service = service;
    }
    async create(req, dto, res) {
        const resData = (0, class_transformer_1.plainToClass)(visitordto_1.VisitorDTO, await this.service.createOne(req, dto));
        res.status(common_1.HttpStatus.OK).send(resData);
    }
    async get(req, res) {
        try {
            const resData = (0, class_transformer_1.plainToClass)(visitordto_1.VisitorDTO, await this.service.getOne(req));
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
    __metadata("design:paramtypes", [Object, visitor_entity_1.Visitor, Object]),
    __metadata("design:returntype", Promise)
], VisitorCrudController.prototype, "create", null);
__decorate([
    (0, crud_1.Override)('getOneBase'),
    openapi.ApiResponse({ status: 200 }),
    __param(0, (0, crud_1.ParsedRequest)()),
    __param(1, (0, common_1.Res)()),
    __metadata("design:type", Function),
    __metadata("design:paramtypes", [Object, Object]),
    __metadata("design:returntype", Promise)
], VisitorCrudController.prototype, "get", null);
__decorate([
    (0, crud_1.Override)('deleteOneBase'),
    openapi.ApiResponse({ status: 200 }),
    __param(0, (0, crud_1.ParsedRequest)()),
    __param(1, (0, common_1.Param)('id')),
    __param(2, (0, common_1.Res)()),
    __metadata("design:type", Function),
    __metadata("design:paramtypes", [Object, Number, Object]),
    __metadata("design:returntype", Promise)
], VisitorCrudController.prototype, "delete", null);
VisitorCrudController = __decorate([
    (0, crud_1.Crud)({
        model: {
            type: visitor_entity_1.Visitor,
        },
    }),
    (0, serializer_1.CrudType)(visitor_entity_1.Visitor),
    (0, common_1.Controller)('visitormanagement/v1/visitor'),
    (0, swagger_1.ApiTags)('Visitor'),
    __metadata("design:paramtypes", [visitor_crud_service_1.VisitorCrudService])
], VisitorCrudController);
exports.VisitorCrudController = VisitorCrudController;
//# sourceMappingURL=visitor.crud.controller.js.map