import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { TypeOrmCrudService } from '@nestjsx/crud-typeorm';
import { Repository } from 'typeorm';
import { AccessCodeDTO } from '../dto/accessCode';
import { AccessCodeResponse } from '../dto/accessCodeResponse';
import { AccessCode } from '../model/entities/accessCode.entity';
import { plainToClass } from 'class-transformer';
import { ComposedCTO } from '../dto/composedCto';
import { Queue } from 'src/app/queue/model/entities/queue.entity';
import { VisitorDTO } from 'src/app/visitor/dto/visitordto';

@Injectable()
export class AccessCodeCrudService extends TypeOrmCrudService<AccessCode> {
  constructor(@InjectRepository(AccessCode) private repoCode: Repository<AccessCode>) {
    super(repoCode);
  }

  async getOneMod(id: number): Promise<ComposedCTO> {
    const accessCode = await this.repoCode.findOne({
      where: { id: id },
      relations: ['visitor', 'queue'],
    });

    const cto = new ComposedCTO();
    cto.accessCode = plainToClass(AccessCodeResponse, accessCode);
    cto.queue = plainToClass(Queue, accessCode?.queue);
    cto.visitor = plainToClass(VisitorDTO, accessCode?.visitor);
    return cto;
  }

  async createOneMod(dto: AccessCodeDTO): Promise<AccessCodeResponse | undefined> {
    const existVisitor = await this.repoCode.findOne({ where: { queueId: dto.queueId, visitorId: dto.visitorId } });

    if (existVisitor != undefined) {
      const accessCode = plainToClass(AccessCodeResponse, existVisitor);
      accessCode.ticketNumber = this.generateTicketCode(accessCode.id);
      return accessCode;
    }

    const accessCode: AccessCode = new AccessCode();
    accessCode.creationTime = new Date();
    accessCode.startTime = new Date();
    accessCode.queueId = dto.queueId;
    accessCode.visitorId = dto.visitorId;
    accessCode.modificationCounter = 0;
    const insertAccessCode = plainToClass(AccessCodeResponse, await this.repoCode.save(accessCode));
    insertAccessCode.ticketNumber = this.generateTicketCode(insertAccessCode.id);
    return insertAccessCode;
  }

  async deleteOneMod(id: number): Promise<void> {
    await this.repoCode.delete(id);
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
