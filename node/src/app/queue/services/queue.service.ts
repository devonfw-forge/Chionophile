import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { Criteria } from '../dto/criteria';
import { QueueResponseDTO } from '../dto/queueResponseDto';
import { Queue } from '../model/entities/queue.entity';

@Injectable()
export class QueueService {
  constructor(@InjectRepository(Queue) private repoQueue: Repository<Queue>) {}
  async searchCriteria(crit: Criteria): Promise<QueueResponseDTO> {
    const response = new QueueResponseDTO();
    response.pageable = crit.pageable;
    response.content = await this.repoQueue.find({ active: crit.active });
    response.totalElements = response.content.length;
    return response;
  }
}
