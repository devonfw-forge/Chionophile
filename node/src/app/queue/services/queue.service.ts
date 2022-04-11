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
    console.log(crit);
    const response = new QueueResponseDTO();
    response.pageable = crit.pageable;
    if (crit.hasOwnProperty("active")){
      response.content = await this.repoQueue.find({
                                                    skip: crit.pageable.pageNumber * crit.pageable.pageSize,
                                                    take: crit.pageable.pageSize, 
                                                    where: {
                                                    active: crit.active }});
    } else{
      response.content = await this.repoQueue.find({
      skip: crit.pageable.pageNumber * crit.pageable.pageSize,
      take: crit.pageable.pageSize});
    }

    response.totalElements = response.content.length;
    return response;
  }
}
