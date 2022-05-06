import { CrudRequest } from '@nestjsx/crud';
import { Queue } from '../model/entities/queue.entity';
import { QueueCrudService } from '../services/queue.crud.service';
import { Response } from 'express';
export declare class QueueCrudController {
    service: QueueCrudService;
    constructor(service: QueueCrudService);
    create(req: CrudRequest, dto: Queue, res: Response): Promise<void>;
    get(req: CrudRequest, res: Response): Promise<void>;
    delete(req: CrudRequest, id: number, res: Response): Promise<void>;
}
