import { TypeOrmCrudService } from '@nestjsx/crud-typeorm';
import { Repository } from 'typeorm';
import { Queue } from '../model/entities/queue.entity';
export declare class QueueCrudService extends TypeOrmCrudService<Queue> {
    constructor(repo: Repository<Queue>);
}
