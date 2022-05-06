import { Repository } from 'typeorm';
import { Criteria } from '../dto/criteria';
import { QueueResponseDTO } from '../dto/queueResponseDto';
import { Queue } from '../model/entities/queue.entity';
export declare class QueueService {
    private repoQueue;
    constructor(repoQueue: Repository<Queue>);
    searchCriteria(crit: Criteria): Promise<QueueResponseDTO>;
}
