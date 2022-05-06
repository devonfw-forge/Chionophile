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
exports.QueueService = void 0;
const common_1 = require("@nestjs/common");
const typeorm_1 = require("@nestjs/typeorm");
const typeorm_2 = require("typeorm");
const queueResponseDto_1 = require("../dto/queueResponseDto");
const queue_entity_1 = require("../model/entities/queue.entity");
let QueueService = class QueueService {
    constructor(repoQueue) {
        this.repoQueue = repoQueue;
    }
    async searchCriteria(crit) {
        let query_params = {};
        let criterium;
        for (criterium in crit) {
            if (crit.hasOwnProperty(criterium) && criterium != 'pageable' && crit[criterium] != undefined) {
                query_params[criterium] = crit[criterium];
            }
        }
        const response = new queueResponseDto_1.QueueResponseDTO();
        response.pageable = crit.pageable;
        if (Object.keys(query_params).length != 0) {
            response.content = await this.repoQueue.find({
                skip: crit.pageable.pageNumber * crit.pageable.pageSize,
                take: crit.pageable.pageSize,
                where: query_params,
            });
        }
        else {
            response.content = await this.repoQueue.find({
                skip: crit.pageable.pageNumber * crit.pageable.pageSize,
                take: crit.pageable.pageSize,
            });
        }
        response.totalElements = response.content.length;
        return response;
    }
};
QueueService = __decorate([
    (0, common_1.Injectable)(),
    __param(0, (0, typeorm_1.InjectRepository)(queue_entity_1.Queue)),
    __metadata("design:paramtypes", [typeorm_2.Repository])
], QueueService);
exports.QueueService = QueueService;
//# sourceMappingURL=queue.service.js.map