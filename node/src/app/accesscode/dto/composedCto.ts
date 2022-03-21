import { AccessCode } from '../model/entities/accessCode.entity';
import { Queue } from '../../queue/model/entities/queue.entity';
import { VisitorResponseDTO } from '../../visitor/dto/visitorResponseDto';

export class ComposedCTO {
  accessCode: AccessCode;
  queue: Queue;
  visitor: VisitorResponseDTO;
}
