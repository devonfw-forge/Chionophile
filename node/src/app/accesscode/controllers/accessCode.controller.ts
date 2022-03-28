import { Controller, Post, Body, HttpCode } from '@nestjs/common';
import { AccessCodeSearchDTO } from '../dto/accessCodeSearchDto';
import { Criteria } from '../dto/criteria';
import { AccessCodeService } from '../services/accessCode.service';

@Controller('accesscodemanagement/v1/accesscode/cto/search')
export class AccessCodeController {
  constructor(public readonly accessCode: AccessCodeService) {}

  @Post()
  @HttpCode(200)
  searchCode(@Body() crit: Criteria): Promise<AccessCodeSearchDTO> {
    return this.accessCode.searchCriteria(crit);
  }
}
