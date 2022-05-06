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
exports.VisitorService = void 0;
const common_1 = require("@nestjs/common");
const typeorm_1 = require("@nestjs/typeorm");
const class_transformer_1 = require("class-transformer");
const typeorm_2 = require("typeorm");
const visitordto_1 = require("../dto/visitordto");
const visitorResponseDto_1 = require("../dto/visitorResponseDto");
const visitor_entity_1 = require("../model/entities/visitor.entity");
let VisitorService = class VisitorService {
    constructor(repo) {
        this.repo = repo;
    }
    async searchCriteria(crit) {
        let query_params = {};
        let criterium;
        for (criterium in crit) {
            if (crit.hasOwnProperty(criterium) && criterium != 'pageable' && crit[criterium] != undefined) {
                query_params[criterium] = crit[criterium];
            }
        }
        const response = new visitorResponseDto_1.VisitorResponseDTO();
        response.pageable = crit.pageable;
        if (Object.keys(query_params).length != 0) {
            response.content = (await this.repo.find({
                skip: crit.pageable.pageNumber * crit.pageable.pageSize,
                take: crit.pageable.pageSize,
                where: query_params,
            })).map(visitor => (0, class_transformer_1.plainToClass)(visitordto_1.VisitorDTO, visitor));
        }
        else {
            response.content = (await this.repo.find({
                skip: crit.pageable.pageNumber * crit.pageable.pageSize,
                take: crit.pageable.pageSize,
            })).map((visitor) => (0, class_transformer_1.plainToClass)(visitordto_1.VisitorDTO, visitor));
        }
        response.totalElements = response.content.length;
        return response;
    }
};
VisitorService = __decorate([
    (0, common_1.Injectable)(),
    __param(0, (0, typeorm_1.InjectRepository)(visitor_entity_1.Visitor)),
    __metadata("design:paramtypes", [typeorm_2.Repository])
], VisitorService);
exports.VisitorService = VisitorService;
//# sourceMappingURL=visitor.service.js.map