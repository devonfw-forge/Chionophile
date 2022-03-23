import { Module } from '@nestjs/common';
import { Queue } from './model/entities/queue.entity';
import { TypeOrmModule } from '@nestjs/typeorm';
import { QueueCrudService } from './services/queue.crud.service';
import { QueueCrudController } from './controllers/queue.crud.controller';
import { QueueController } from './controllers/queue.controller';
import { QueueService } from './services/queue.service';

@Module({
  imports: [TypeOrmModule.forFeature([Queue])],
  providers: [QueueService, QueueCrudService],
  controllers: [QueueController, QueueCrudController],
})
export class QueueModule {}
