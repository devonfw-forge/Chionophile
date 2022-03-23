import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { Queue } from '../../queue/model/entities/queue.entity';
import { Visitor } from '../../visitor/model/entities/visitor.entity';
import { AccessCodeSearchDTO } from '../dto/accessCodeSearchDto';
import { ComposedCTO } from '../dto/composedCto';
import { Criteria } from '../dto/criteria';
import { AccessCode } from '../model/entities/accessCode.entity';
import { plainToClass } from 'class-transformer';
import { VisitorResponseDTO } from '../../visitor/dto/visitorResponseDto';

@Injectable()
export class AccessCodeService {
  constructor(
    @InjectRepository(AccessCode) private repoCode: Repository<AccessCode>,
    @InjectRepository(Queue) private repoQueue: Repository<Queue>,
    @InjectRepository(Visitor) private repoVisitor: Repository<Visitor>,
  ) {}

  async searchCriteria(crit: Criteria): Promise<AccessCodeSearchDTO> {
    const response: AccessCodeSearchDTO = new AccessCodeSearchDTO();
    response.pageable = crit.pageable;

    if (crit.hasOwnProperty('visitorId')) {
      response.content = (
        await this.repoCode.find({
          skip: crit.pageable.pageNumber * crit.pageable.pageSize,
          take: crit.pageable.pageSize,
          where: { visitorId: crit.visitorId },
        })
      ).map(code => {
        const cto = new ComposedCTO();
        cto.accessCode = code;
        return cto;
      });
    } else {
      response.content = (
        await this.repoCode.find({
          skip: crit.pageable.pageNumber * crit.pageable.pageSize,
          take: crit.pageable.pageSize,
        })
      ).map(code => {
        const cto = new ComposedCTO();
        cto.accessCode = code;
        return cto;
      });
    }

    for (const item of response.content) {
      if (item != undefined) {
        item.queue = plainToClass(Queue, await this.repoQueue.findOne({ where: { id: item.accessCode?.queueId } }));
        item.visitor = plainToClass(
          VisitorResponseDTO,
          await this.repoVisitor.findOne({ where: { id: item.accessCode?.visitorId } }),
        );
      }
    }

    response.totalElements = response.content.length;
    return response;
  }
}
