import { Module } from '@nestjs/common';
import { Visitor } from './model/entities/visitor.entity';
import { TypeOrmModule } from '@nestjs/typeorm';
import { VisitorCrudService } from './services/visitor.crud.service';
import { VisitorCrudController } from './controllers/visitor.crud.controller';
import { VisitorService } from './services/visitor.service';
import { VisitorController } from './controllers/visitor.controller';

@Module({
  imports: [TypeOrmModule.forFeature([Visitor])],
  providers: [VisitorCrudService, VisitorService],
  controllers: [VisitorCrudController, VisitorController],
})
export class VisitorModule {}
