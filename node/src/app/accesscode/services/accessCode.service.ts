import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { plainToClass } from 'class-transformer';
import { Queue } from 'src/app/queue/model/entities/queue.entity';
import { VisitorDTO } from 'src/app/visitor/dto/visitordto';
import { Repository } from 'typeorm';
import { AccessCodeResponse } from '../dto/accessCodeResponse';
import { AccessCodeSearchDTO } from '../dto/accessCodeSearchDto';
import { ComposedCTO } from '../dto/composedCto';
import { Criteria } from '../dto/criteria';
import { AccessCode } from '../model/entities/accessCode.entity';

@Injectable()
export class AccessCodeService {
  constructor(
    @InjectRepository(AccessCode) private repoCode: Repository<AccessCode>
  ) {}

  async searchCriteria(crit: Criteria): Promise<AccessCodeSearchDTO> {
    let query_params: any = {}
    let criterium: keyof Criteria
    for (criterium in crit) {
      if (crit.hasOwnProperty(criterium) && criterium != "pageable" && crit[criterium] != undefined) {
        if (criterium == "ticketNumber"){
          query_params["id"] = parseInt(crit[criterium].slice(1))
        } else{
          query_params[criterium] = crit[criterium];
        }
      }
    }

    const response: AccessCodeSearchDTO = new AccessCodeSearchDTO();
    if ( Object.keys(query_params).length != 0 ) {
      response.content = (
        await this.repoCode.find({
          skip: crit.pageable.pageNumber * crit.pageable.pageSize,
          take: crit.pageable.pageSize,
          where: query_params,
          relations: ['visitor', 'queue']
        })
      ).map(code => {
        const cto = new ComposedCTO();
        cto.accessCode = plainToClass(AccessCodeResponse, code);
        cto.queue = plainToClass(Queue, code.queue);
        cto.visitor = plainToClass(VisitorDTO, code.visitor);
        return cto;
      });
    } else {
      response.content = (
        await this.repoCode.find({
          skip: crit.pageable.pageNumber * crit.pageable.pageSize,
          take: crit.pageable.pageSize,
          relations: ['visitor', 'queue']
        })
      ).map(code => {
        const cto = new ComposedCTO();
        cto.accessCode = plainToClass(AccessCodeResponse, code);
        cto.accessCode.ticketNumber = this.generateTicketCode(cto.accessCode.id);
        cto.queue = code.queue;
        cto.visitor = code.visitor;
        return cto;
      });
    }

    response.pageable = crit.pageable;
    response.totalElements = response.content.length;
    return response;
  }

  private generateTicketCode(lastTicketDigit: number): string {
    const newTicketDigit: number = lastTicketDigit + 1;
    let newTicketCode: string = newTicketDigit.toString();
    if (newTicketDigit == 1000) {
      newTicketCode = 'Q000';
    } else {
      while (newTicketCode.length < 3) {
        newTicketCode = '0' + newTicketCode;
      }
      newTicketCode = 'Q' + newTicketCode;
    }
    return newTicketCode;
  }
}
