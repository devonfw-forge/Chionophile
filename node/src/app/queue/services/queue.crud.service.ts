import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { TypeOrmCrudService } from '@nestjsx/crud-typeorm';
import { Repository } from 'typeorm';
import { Queue } from '../model/entities/queue.entity';

@Injectable()
export class QueueCrudService extends TypeOrmCrudService<Queue> {
  constructor(@InjectRepository(Queue) repo: Repository<Queue>) {
    super(repo);
  }
}
