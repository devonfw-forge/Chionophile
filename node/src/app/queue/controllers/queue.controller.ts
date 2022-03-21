import { Body, Controller, Post } from '@nestjs/common';
import { Criteria } from '../dto/criteria';
import { QueueResponseDTO } from '../dto/queueResponseDto';
import { QueueService } from '../services/queue.service';

@Controller('queuemanagement/v1/queue/search')
export class QueueController {
  constructor(public readonly queueServices: QueueService) {}

  @Post()
  searchCode(@Body() crit: Criteria): Promise<QueueResponseDTO> {
    return this.queueServices.searchCriteria(crit);
  }
}
