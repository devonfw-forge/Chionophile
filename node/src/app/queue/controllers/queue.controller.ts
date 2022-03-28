import { Body, Controller, HttpCode, Post } from '@nestjs/common';
import { Criteria } from '../dto/criteria';
import { QueueResponseDTO } from '../dto/queueResponseDto';
import { QueueService } from '../services/queue.service';

@Controller('queuemanagement/v1/queue/search')
export class QueueController {
  constructor(public readonly queueServices: QueueService) {}

  @Post()
  @HttpCode(200)
  searchCode(@Body() crit: Criteria): Promise<QueueResponseDTO> {
    return this.queueServices.searchCriteria(crit);
  }
}
