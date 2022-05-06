import { CrudRequest } from '@nestjsx/crud';
import { AccessCodeCrudService } from '../services/accessCode.crud.service';
import { AccessCodeDTO } from '../dto/accessCode';
import { Response } from 'express';
export declare class AccessCodeCrudController {
    service: AccessCodeCrudService;
    constructor(service: AccessCodeCrudService);
    create(dto: AccessCodeDTO, res: Response): Promise<void>;
    get(id: number, res: Response): Promise<void>;
    delete(req: CrudRequest, id: number, res: Response): Promise<void>;
}
