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
exports.AccessCodeCrudController = void 0;
const openapi = require("@nestjs/swagger");
const common_1 = require("@nestjs/common");
const crud_1 = require("@nestjsx/crud");
const serializer_1 = require("@devon4node/common/serializer");
const accessCode_entity_1 = require("../model/entities/accessCode.entity");
const accessCode_crud_service_1 = require("../services/accessCode.crud.service");
const swagger_1 = require("@nestjs/swagger");
const accessCode_1 = require("../dto/accessCode");
let AccessCodeCrudController = class AccessCodeCrudController {
    constructor(service) {
        this.service = service;
    }
    async create(dto, res) {
        const resData = await this.service.createOneMod(dto);
        res.status(common_1.HttpStatus.OK).send(resData);
    }
    async get(id, res) {
        try {
            const resData = await this.service.getOneMod(id);
            if (resData.accessCode == undefined) {
                throw 'Inexistent AccessCode';
            }
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
    __param(0, (0, crud_1.ParsedBody)()),
    __param(1, (0, common_1.Res)()),
    __metadata("design:type", Function),
    __metadata("design:paramtypes", [accessCode_1.AccessCodeDTO, Object]),
    __metadata("design:returntype", Promise)
], AccessCodeCrudController.prototype, "create", null);
__decorate([
    (0, crud_1.Override)('getOneBase'),
    openapi.ApiResponse({ status: 200 }),
    __param(0, (0, common_1.Param)('id')),
    __param(1, (0, common_1.Res)()),
    __metadata("design:type", Function),
    __metadata("design:paramtypes", [Number, Object]),
    __metadata("design:returntype", Promise)
], AccessCodeCrudController.prototype, "get", null);
__decorate([
    (0, crud_1.Override)('deleteOneBase'),
    openapi.ApiResponse({ status: 200 }),
    __param(0, (0, crud_1.ParsedRequest)()),
    __param(1, (0, common_1.Param)('id')),
    __param(2, (0, common_1.Res)()),
    __metadata("design:type", Function),
    __metadata("design:paramtypes", [Object, Number, Object]),
    __metadata("design:returntype", Promise)
], AccessCodeCrudController.prototype, "delete", null);
AccessCodeCrudController = __decorate([
    (0, crud_1.Crud)({
        model: {
            type: accessCode_entity_1.AccessCode,
        },
    }),
    (0, serializer_1.CrudType)(accessCode_entity_1.AccessCode),
    (0, common_1.Controller)('/accesscodemanagement/v1/accesscode'),
    (0, swagger_1.ApiTags)('AccessCode'),
    __metadata("design:paramtypes", [accessCode_crud_service_1.AccessCodeCrudService])
], AccessCodeCrudController);
exports.AccessCodeCrudController = AccessCodeCrudController;
//# sourceMappingURL=accessCode.crud.controller.js.map