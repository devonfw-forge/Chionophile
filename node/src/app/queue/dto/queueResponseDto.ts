import { Pageable } from '../../shared/interfaces/pageable';
import { Queue } from '../model/entities/queue.entity';

export class QueueResponseDTO {
  content: Queue[];
  pageable: Pageable;
  totalElements: number;
}
