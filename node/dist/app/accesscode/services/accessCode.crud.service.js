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
exports.AccessCodeCrudService = void 0;
const common_1 = require("@nestjs/common");
const typeorm_1 = require("@nestjs/typeorm");
const crud_typeorm_1 = require("@nestjsx/crud-typeorm");
const typeorm_2 = require("typeorm");
const accessCodeResponse_1 = require("../dto/accessCodeResponse");
const accessCode_entity_1 = require("../model/entities/accessCode.entity");
const class_transformer_1 = require("class-transformer");
const composedCto_1 = require("../dto/composedCto");
const queue_entity_1 = require("../../queue/model/entities/queue.entity");
const visitordto_1 = require("../../visitor/dto/visitordto");
let AccessCodeCrudService = class AccessCodeCrudService extends crud_typeorm_1.TypeOrmCrudService {
    constructor(repoCode) {
        super(repoCode);
        this.repoCode = repoCode;
    }
    async getOneMod(id) {
        const accessCode = await this.repoCode.findOne({
            where: { id: id },
            relations: ['visitor', 'queue'],
        });
        const cto = new composedCto_1.ComposedCTO();
        cto.accessCode = (0, class_transformer_1.plainToClass)(accessCodeResponse_1.AccessCodeResponse, accessCode);
        cto.queue = (0, class_transformer_1.plainToClass)(queue_entity_1.Queue, accessCode === null || accessCode === void 0 ? void 0 : accessCode.queue);
        cto.visitor = (0, class_transformer_1.plainToClass)(visitordto_1.VisitorDTO, accessCode === null || accessCode === void 0 ? void 0 : accessCode.visitor);
        return cto;
    }
    async createOneMod(dto) {
        const existVisitor = await this.repoCode.findOne({ where: { queueId: dto.queueId, visitorId: dto.visitorId } });
        if (existVisitor != undefined) {
            const accessCode = (0, class_transformer_1.plainToClass)(accessCodeResponse_1.AccessCodeResponse, existVisitor);
            accessCode.ticketNumber = this.generateTicketCode(accessCode.id);
            return accessCode;
        }
        const accessCode = new accessCode_entity_1.AccessCode();
        accessCode.creationTime = new Date();
        accessCode.startTime = new Date();
        accessCode.queueId = dto.queueId;
        accessCode.visitorId = dto.visitorId;
        accessCode.modificationCounter = 0;
        const insertAccessCode = (0, class_transformer_1.plainToClass)(accessCodeResponse_1.AccessCodeResponse, await this.repoCode.save(accessCode));
        insertAccessCode.ticketNumber = this.generateTicketCode(insertAccessCode.id);
        return insertAccessCode;
    }
    async deleteOneMod(id) {
        await this.repoCode.delete(id);
    }
    generateTicketCode(lastTicketDigit) {
        const newTicketDigit = lastTicketDigit + 1;
        let newTicketCode = newTicketDigit.toString();
        if (newTicketDigit == 1000) {
            newTicketCode = 'Q000';
        }
        else {
            while (newTicketCode.length < 3) {
                newTicketCode = '0' + newTicketCode;
            }
            newTicketCode = 'Q' + newTicketCode;
        }
        return newTicketCode;
    }
};
AccessCodeCrudService = __decorate([
    (0, common_1.Injectable)(),
    __param(0, (0, typeorm_1.InjectRepository)(accessCode_entity_1.AccessCode)),
    __metadata("design:paramtypes", [typeorm_2.Repository])
], AccessCodeCrudService);
exports.AccessCodeCrudService = AccessCodeCrudService;
//# sourceMappingURL=accessCode.crud.service.js.map