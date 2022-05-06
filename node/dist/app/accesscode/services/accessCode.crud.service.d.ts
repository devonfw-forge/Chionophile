import { TypeOrmCrudService } from '@nestjsx/crud-typeorm';
import { Repository } from 'typeorm';
import { AccessCodeDTO } from '../dto/accessCode';
import { AccessCodeResponse } from '../dto/accessCodeResponse';
import { AccessCode } from '../model/entities/accessCode.entity';
import { ComposedCTO } from '../dto/composedCto';
export declare class AccessCodeCrudService extends TypeOrmCrudService<AccessCode> {
    private repoCode;
    constructor(repoCode: Repository<AccessCode>);
    getOneMod(id: number): Promise<ComposedCTO>;
    createOneMod(dto: AccessCodeDTO): Promise<AccessCodeResponse | undefined>;
    deleteOneMod(id: number): Promise<void>;
    private generateTicketCode;
}
