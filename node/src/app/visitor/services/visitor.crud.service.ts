import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { TypeOrmCrudService } from '@nestjsx/crud-typeorm';
import { Repository } from 'typeorm';
import { Visitor } from '../model/entities/visitor.entity';

@Injectable()
export class VisitorCrudService extends TypeOrmCrudService<Visitor> {
  constructor(@InjectRepository(Visitor) repo: Repository<Visitor>) {
    super(repo);
  }
}
