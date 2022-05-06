"use strict";
var __decorate = (this && this.__decorate) || function (decorators, target, key, desc) {
    var c = arguments.length, r = c < 3 ? target : desc === null ? desc = Object.getOwnPropertyDescriptor(target, key) : desc, d;
    if (typeof Reflect === "object" && typeof Reflect.decorate === "function") r = Reflect.decorate(decorators, target, key, desc);
    else for (var i = decorators.length - 1; i >= 0; i--) if (d = decorators[i]) r = (c < 3 ? d(r) : c > 3 ? d(target, key, r) : d(target, key)) || r;
    return c > 3 && r && Object.defineProperty(target, key, r), r;
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.QueueModule = void 0;
const common_1 = require("@nestjs/common");
const queue_entity_1 = require("./model/entities/queue.entity");
const typeorm_1 = require("@nestjs/typeorm");
const queue_crud_service_1 = require("./services/queue.crud.service");
const queue_crud_controller_1 = require("./controllers/queue.crud.controller");
const queue_controller_1 = require("./controllers/queue.controller");
const queue_service_1 = require("./services/queue.service");
const accessCode_entity_1 = require("../accesscode/model/entities/accessCode.entity");
let QueueModule = class QueueModule {
};
QueueModule = __decorate([
    (0, common_1.Module)({
        imports: [typeorm_1.TypeOrmModule.forFeature([queue_entity_1.Queue, accessCode_entity_1.AccessCode])],
        providers: [queue_service_1.QueueService, queue_crud_service_1.QueueCrudService],
        controllers: [queue_controller_1.QueueController, queue_crud_controller_1.QueueCrudController],
    })
], QueueModule);
exports.QueueModule = QueueModule;
//# sourceMappingURL=queue.module.js.map