import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { plainToClass } from 'class-transformer';
import { Repository } from 'typeorm';
import { Criteria } from '../dto/criteria';
import { VisitorDTO } from '../dto/visitordto';
import { VisitorResponseDTO } from '../dto/visitorResponseDto';
import { Visitor } from '../model/entities/visitor.entity';

@Injectable()
export class VisitorService {
  constructor(@InjectRepository(Visitor) private repo: Repository<Visitor>) {}

  async searchCriteria(crit: Criteria): Promise<VisitorResponseDTO> {
    let query_params: any = {};
    let criterium: keyof Criteria;
    for (criterium in crit) {
      if (crit.hasOwnProperty(criterium) && criterium != 'pageable' && crit[criterium] != undefined) {
        query_params[criterium] = crit[criterium];
      }
    }

    const response: VisitorResponseDTO = new VisitorResponseDTO();
    response.pageable = crit.pageable;
    if (Object.keys(query_params).length != 0) {
      response.content = (
        await this.repo.find({
          skip: crit.pageable.pageNumber * crit.pageable.pageSize,
          take: crit.pageable.pageSize,
          where: query_params,
        })
      ).map(visitor => plainToClass(VisitorDTO, visitor));
    } else {
      response.content = (
        await this.repo.find({
          skip: crit.pageable.pageNumber * crit.pageable.pageSize,
          take: crit.pageable.pageSize,
        })
      ).map((visitor: any) => plainToClass(VisitorDTO, visitor));
    }
    response.totalElements = response.content.length;

    return response;
  }
}
