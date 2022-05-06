import { CrudRequest } from '@nestjsx/crud';
import { Visitor } from '../model/entities/visitor.entity';
import { VisitorCrudService } from '../services/visitor.crud.service';
import { Response } from 'express';
export declare class VisitorCrudController {
    service: VisitorCrudService;
    constructor(service: VisitorCrudService);
    create(req: CrudRequest, dto: Visitor, res: Response): Promise<void>;
    get(req: CrudRequest, res: Response): Promise<void>;
    delete(req: CrudRequest, id: number, res: Response): Promise<void>;
}
