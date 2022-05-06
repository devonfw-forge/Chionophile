import { Repository } from 'typeorm';
import { Criteria } from '../dto/criteria';
import { VisitorResponseDTO } from '../dto/visitorResponseDto';
import { Visitor } from '../model/entities/visitor.entity';
export declare class VisitorService {
    private repo;
    constructor(repo: Repository<Visitor>);
    searchCriteria(crit: Criteria): Promise<VisitorResponseDTO>;
}
