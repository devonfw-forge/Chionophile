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
exports.AccessCodeService = void 0;
const common_1 = require("@nestjs/common");
const typeorm_1 = require("@nestjs/typeorm");
const class_transformer_1 = require("class-transformer");
const queue_entity_1 = require("../../queue/model/entities/queue.entity");
const visitordto_1 = require("../../visitor/dto/visitordto");
const typeorm_2 = require("typeorm");
const accessCodeResponse_1 = require("../dto/accessCodeResponse");
const accessCodeSearchDto_1 = require("../dto/accessCodeSearchDto");
const composedCto_1 = require("../dto/composedCto");
const accessCode_entity_1 = require("../model/entities/accessCode.entity");
let AccessCodeService = class AccessCodeService {
    constructor(repoCode) {
        this.repoCode = repoCode;
    }
    async searchCriteria(crit) {
        let query_params = {};
        let criterium;
        for (criterium in crit) {
            if (crit.hasOwnProperty(criterium) && criterium != 'pageable' && crit[criterium] != undefined) {
                if (criterium == 'ticketNumber') {
                    query_params['id'] = parseInt(crit[criterium].slice(1));
                }
                else {
                    query_params[criterium] = crit[criterium];
                }
            }
        }
        const response = new accessCodeSearchDto_1.AccessCodeSearchDTO();
        if (Object.keys(query_params).length != 0) {
            response.content = (await this.repoCode.find({
                skip: crit.pageable.pageNumber * crit.pageable.pageSize,
                take: crit.pageable.pageSize,
                where: query_params,
                relations: ['visitor', 'queue'],
            })).map(code => {
                const cto = new composedCto_1.ComposedCTO();
                cto.accessCode = (0, class_transformer_1.plainToClass)(accessCodeResponse_1.AccessCodeResponse, code);
                cto.queue = (0, class_transformer_1.plainToClass)(queue_entity_1.Queue, code.queue);
                cto.visitor = (0, class_transformer_1.plainToClass)(visitordto_1.VisitorDTO, code.visitor);
                return cto;
            });
        }
        else {
            response.content = (await this.repoCode.find({
                skip: crit.pageable.pageNumber * crit.pageable.pageSize,
                take: crit.pageable.pageSize,
                relations: ['visitor', 'queue'],
            })).map(code => {
                const cto = new composedCto_1.ComposedCTO();
                cto.accessCode = (0, class_transformer_1.plainToClass)(accessCodeResponse_1.AccessCodeResponse, code);
                cto.accessCode.ticketNumber = this.generateTicketCode(cto.accessCode.id);
                cto.queue = code.queue;
                cto.visitor = code.visitor;
                return cto;
            });
        }
        response.pageable = crit.pageable;
        response.totalElements = response.content.length;
        return response;
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
AccessCodeService = __decorate([
    (0, common_1.Injectable)(),
    __param(0, (0, typeorm_1.InjectRepository)(accessCode_entity_1.AccessCode)),
    __metadata("design:paramtypes", [typeorm_2.Repository])
], AccessCodeService);
exports.AccessCodeService = AccessCodeService;
//# sourceMappingURL=accessCode.service.js.map