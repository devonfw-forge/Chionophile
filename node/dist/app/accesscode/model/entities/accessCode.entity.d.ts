import { Visitor } from '../../../visitor/model/entities/visitor.entity';
import { Queue } from '../../../queue/model/entities/queue.entity';
export declare class AccessCode {
    id: number;
    modificationCounter?: number;
    creationTime?: Date;
    startTime?: Date;
    endTime?: Date;
    visitorId?: number;
    queueId?: number;
    visitor: Visitor;
    queue: Queue;
}
