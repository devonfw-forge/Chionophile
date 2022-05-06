import { TypeOrmCrudService } from '@nestjsx/crud-typeorm';
import { Repository } from 'typeorm';
import { Visitor } from '../model/entities/visitor.entity';
export declare class VisitorCrudService extends TypeOrmCrudService<Visitor> {
    constructor(repo: Repository<Visitor>);
}
