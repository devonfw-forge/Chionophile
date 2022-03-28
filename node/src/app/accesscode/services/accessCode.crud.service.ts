import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { TypeOrmCrudService } from '@nestjsx/crud-typeorm';
import { Repository } from 'typeorm';
import { AccessCodeDTO } from '../dto/accessCode';
import { AccessCodeResponse } from '../dto/accessCodeResponse';
import { AccessCode } from '../model/entities/accessCode.entity';
import { plainToClass } from 'class-transformer';

@Injectable()
export class AccessCodeCrudService extends TypeOrmCrudService<AccessCode> {
  constructor(
    @InjectRepository(AccessCode) private repoCode: Repository<AccessCode>,
  ) {
    super(repoCode);
  }

  async createOneMod(dto: AccessCodeDTO): Promise<AccessCodeResponse | undefined> {
    const existVisitor = await this.repoCode.findOne({ where: { queueId: dto.queueId, visitorId: dto.visitorId } });

    if (existVisitor != undefined) {
      return plainToClass(AccessCodeResponse, existVisitor);
    }

    const lastTicket: number = await this.repoCode.count({ where: { queueId: dto.queueId } });
    const ticketCode: string = this.generateTicketCode(lastTicket);
    const accessCode: AccessCode = new AccessCode();
    accessCode.creationTime = new Date();
    accessCode.startTime = new Date();
    accessCode.queueId = dto.queueId;
    accessCode.visitorId = dto.visitorId;
    accessCode.modificationCounter = 0;
    accessCode.ticketNumber = ticketCode;
    const insertAccessCode = plainToClass(AccessCodeResponse, await this.repoCode.save(accessCode));

    return insertAccessCode;
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

  async deleteOneMod(id: number): Promise<void> {
    await this.repoCode.delete(id);
  }
}
