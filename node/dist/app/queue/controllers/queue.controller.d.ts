import { Criteria } from '../dto/criteria';
import { QueueResponseDTO } from '../dto/queueResponseDto';
import { QueueService } from '../services/queue.service';
export declare class QueueController {
    readonly queueServices: QueueService;
    constructor(queueServices: QueueService);
    searchCode(crit: Criteria): Promise<QueueResponseDTO>;
}
