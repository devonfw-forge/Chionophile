"use strict";
var __decorate = (this && this.__decorate) || function (decorators, target, key, desc) {
    var c = arguments.length, r = c < 3 ? target : desc === null ? desc = Object.getOwnPropertyDescriptor(target, key) : desc, d;
    if (typeof Reflect === "object" && typeof Reflect.decorate === "function") r = Reflect.decorate(decorators, target, key, desc);
    else for (var i = decorators.length - 1; i >= 0; i--) if (d = decorators[i]) r = (c < 3 ? d(r) : c > 3 ? d(target, key, r) : d(target, key)) || r;
    return c > 3 && r && Object.defineProperty(target, key, r), r;
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.AccessCodeModule = void 0;
const common_1 = require("@nestjs/common");
const accessCode_entity_1 = require("./model/entities/accessCode.entity");
const typeorm_1 = require("@nestjs/typeorm");
const accessCode_crud_service_1 = require("./services/accessCode.crud.service");
const accessCode_crud_controller_1 = require("./controllers/accessCode.crud.controller");
const accessCode_service_1 = require("./services/accessCode.service");
const accessCode_controller_1 = require("./controllers/accessCode.controller");
const visitor_entity_1 = require("../visitor/model/entities/visitor.entity");
const queue_entity_1 = require("../queue/model/entities/queue.entity");
let AccessCodeModule = class AccessCodeModule {
};
AccessCodeModule = __decorate([
    (0, common_1.Module)({
        imports: [typeorm_1.TypeOrmModule.forFeature([accessCode_entity_1.AccessCode, visitor_entity_1.Visitor, queue_entity_1.Queue])],
        providers: [accessCode_crud_service_1.AccessCodeCrudService, accessCode_service_1.AccessCodeService],
        controllers: [accessCode_crud_controller_1.AccessCodeCrudController, accessCode_controller_1.AccessCodeController],
    })
], AccessCodeModule);
exports.AccessCodeModule = AccessCodeModule;
//# sourceMappingURL=accessCode.module.js.map