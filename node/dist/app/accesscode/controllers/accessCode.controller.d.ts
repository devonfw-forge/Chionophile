import { AccessCodeSearchDTO } from '../dto/accessCodeSearchDto';
import { Criteria } from '../dto/criteria';
import { AccessCodeService } from '../services/accessCode.service';
export declare class AccessCodeController {
    readonly accessCode: AccessCodeService;
    constructor(accessCode: AccessCodeService);
    searchCode(crit: Criteria): Promise<AccessCodeSearchDTO>;
}
