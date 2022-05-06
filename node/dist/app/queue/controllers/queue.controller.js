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
exports.QueueController = void 0;
const openapi = require("@nestjs/swagger");
const common_1 = require("@nestjs/common");
const criteria_1 = require("../dto/criteria");
const queue_service_1 = require("../services/queue.service");
let QueueController = class QueueController {
    constructor(queueServices) {
        this.queueServices = queueServices;
    }
    searchCode(crit) {
        return this.queueServices.searchCriteria(crit);
    }
};
__decorate([
    (0, common_1.Post)(),
    (0, common_1.HttpCode)(200),
    openapi.ApiResponse({ status: 200, type: require("../dto/queueResponseDto").QueueResponseDTO }),
    __param(0, (0, common_1.Body)()),
    __metadata("design:type", Function),
    __metadata("design:paramtypes", [criteria_1.Criteria]),
    __metadata("design:returntype", Promise)
], QueueController.prototype, "searchCode", null);
QueueController = __decorate([
    (0, common_1.Controller)('queuemanagement/v1/queue/search'),
    __metadata("design:paramtypes", [queue_service_1.QueueService])
], QueueController);
exports.QueueController = QueueController;
//# sourceMappingURL=queue.controller.js.map