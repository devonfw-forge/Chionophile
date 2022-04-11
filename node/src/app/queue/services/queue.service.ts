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
    let query_params: any = {}
    let criterium: keyof Criteria
    for (criterium in crit) {
      if (crit.hasOwnProperty(criterium) && criterium != "pageable" && crit[criterium] != undefined) {
          query_params[criterium] = crit[criterium];
      }
    }

    const response = new QueueResponseDTO();
    response.pageable = crit.pageable;
    if ( Object.keys(query_params).length != 0 ){
      response.content = await this.repoQueue.find({
                                                    skip: crit.pageable.pageNumber * crit.pageable.pageSize,
                                                    take: crit.pageable.pageSize, 
                                                    where: query_params});
    } else{
      response.content = await this.repoQueue.find({
      skip: crit.pageable.pageNumber * crit.pageable.pageSize,
      take: crit.pageable.pageSize});
    }

    response.totalElements = response.content.length;
    return response;
  }
}
