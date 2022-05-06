import { Repository } from 'typeorm';
import { AccessCodeSearchDTO } from '../dto/accessCodeSearchDto';
import { Criteria } from '../dto/criteria';
import { AccessCode } from '../model/entities/accessCode.entity';
export declare class AccessCodeService {
    private repoCode;
    constructor(repoCode: Repository<AccessCode>);
    searchCriteria(crit: Criteria): Promise<AccessCodeSearchDTO>;
    private generateTicketCode;
}
