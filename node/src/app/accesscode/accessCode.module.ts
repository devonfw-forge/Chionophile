import { Module } from '@nestjs/common';
import { AccessCode } from './model/entities/accessCode.entity';
import { TypeOrmModule } from '@nestjs/typeorm';
import { AccessCodeCrudService } from './services/accessCode.crud.service';
import { AccessCodeCrudController } from './controllers/accessCode.crud.controller';
import { AccessCodeService } from './services/accessCode.service';
import { AccessCodeController } from './controllers/accessCode.controller';
import { Visitor } from '../visitor/model/entities/visitor.entity';
import { Queue } from '../queue/model/entities/queue.entity';

@Module({
  imports: [TypeOrmModule.forFeature([AccessCode, Visitor, Queue])],
  providers: [AccessCodeCrudService, AccessCodeService],
  controllers: [AccessCodeCrudController, AccessCodeController],
})
export class AccessCodeModule {}
