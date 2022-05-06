import { VisitorService } from '../services/visitor.service';
import { Criteria } from '../dto/criteria';
import { VisitorResponseDTO } from '../dto/visitorResponseDto';
export declare class VisitorController {
    readonly visitor: VisitorService;
    constructor(visitor: VisitorService);
    getVisitorByUsername(crit: Criteria): Promise<VisitorResponseDTO>;
}
